package com.example.payment;

import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

import org.springframework.data.domain.Sort;
import org.springframework.data.mongodb.core.MongoTemplate;
import org.springframework.data.mongodb.core.query.Criteria;
import org.springframework.data.mongodb.core.query.Query;

public class PaymentCrossShardQueryService {
    private static final String TID_FIELD = "tid";
    private static final DateTimeFormatter DATE_FORMAT = DateTimeFormatter.BASIC_ISO_DATE;

    private final MongoTemplate mongoTemplate;
    private final String collectionPrefix;

    public PaymentCrossShardQueryService(MongoTemplate mongoTemplate) {
        this(mongoTemplate, "payment_");
    }

    public PaymentCrossShardQueryService(MongoTemplate mongoTemplate, String collectionPrefix) {
        this.mongoTemplate = Objects.requireNonNull(mongoTemplate, "mongoTemplate");
        this.collectionPrefix = Objects.requireNonNull(collectionPrefix, "collectionPrefix");
    }

    /**
     * Page number is 1-based to match common API usage.
     */
    public PageResult<Payment> queryPayments(LocalDate startDate,
                                             LocalDate endDate,
                                             int pageNumber,
                                             int pageSize) {
        return queryPayments(startDate, endDate, pageNumber, pageSize, Sort.Direction.ASC);
    }

    public PageResult<Payment> queryPayments(LocalDate startDate,
                                             LocalDate endDate,
                                             int pageNumber,
                                             int pageSize,
                                             Sort.Direction direction) {
        Objects.requireNonNull(startDate, "startDate");
        Objects.requireNonNull(endDate, "endDate");
        Objects.requireNonNull(direction, "direction");
        if (startDate.isAfter(endDate)) {
            throw new IllegalArgumentException("startDate must be on or before endDate");
        }
        if (pageNumber < 1) {
            throw new IllegalArgumentException("pageNumber must be 1 or greater");
        }
        if (pageSize < 1) {
            throw new IllegalArgumentException("pageSize must be 1 or greater");
        }

        int startYear = startDate.getYear();
        int endYear = endDate.getYear();
        long offset = (long) (pageNumber - 1) * pageSize;

        if (!isCrossShard(startYear, endYear)) {
            return querySingleShard(startYear, startDate, endDate, offset, pageNumber, pageSize, direction);
        }

        return queryCrossShard(startDate, endDate, offset, pageNumber, pageSize, direction);
    }

    private boolean isCrossShard(int startYear, int endYear) {
        return startYear != endYear;
    }

    private PageResult<Payment> querySingleShard(int year,
                                                 LocalDate startDate,
                                                 LocalDate endDate,
                                                 long offset,
                                                 int pageNumber,
                                                 int pageSize,
                                                 Sort.Direction direction) {
        Query dataQuery = buildTidRangeQuery(startDate, endDate)
                .with(Sort.by(direction, TID_FIELD))
                .skip(offset)
                .limit(pageSize);

        List<Payment> items = mongoTemplate.find(dataQuery, Payment.class, collectionName(year));
        long total = mongoTemplate.count(buildTidRangeQuery(startDate, endDate), Payment.class, collectionName(year));

        return new PageResult<>(items, total, pageNumber, pageSize);
    }

    private PageResult<Payment> queryCrossShard(LocalDate startDate,
                                                LocalDate endDate,
                                                long offset,
                                                int pageNumber,
                                                int pageSize,
                                                Sort.Direction direction) {
        List<ShardRange> shards = buildShardRanges(startDate, endDate, direction);

        long total = 0;
        for (ShardRange shard : shards) {
            long count = mongoTemplate.count(
                    buildTidRangeQuery(shard.startDate, shard.endDate),
                    Payment.class,
                    collectionName(shard.year)
            );
            shard.count = count;
            total += count;
        }

        if (total == 0 || offset >= total) {
            return new PageResult<>(new ArrayList<>(), total, pageNumber, pageSize);
        }

        // Because tid is yyyyMMdd prefix, ordering by tid keeps shard results contiguous by year.
        List<Payment> pageItems = new ArrayList<>(pageSize);
        long remainingOffset = offset;
        int remainingLimit = pageSize;

        for (ShardRange shard : shards) {
            if (remainingLimit == 0) {
                break;
            }
            if (shard.count == 0) {
                continue;
            }
            if (remainingOffset >= shard.count) {
                remainingOffset -= shard.count;
                continue;
            }

            long shardSkip = remainingOffset;
            int shardLimit = (int) Math.min((long) remainingLimit, shard.count - shardSkip);

            Query shardQuery = buildTidRangeQuery(shard.startDate, shard.endDate)
                    .with(Sort.by(direction, TID_FIELD))
                    .skip(shardSkip)
                    .limit(shardLimit);

            pageItems.addAll(mongoTemplate.find(shardQuery, Payment.class, collectionName(shard.year)));

            remainingOffset = 0;
            remainingLimit -= shardLimit;
        }

        return new PageResult<>(pageItems, total, pageNumber, pageSize);
    }

    private List<ShardRange> buildShardRanges(LocalDate startDate,
                                              LocalDate endDate,
                                              Sort.Direction direction) {
        int startYear = startDate.getYear();
        int endYear = endDate.getYear();
        List<ShardRange> ranges = new ArrayList<>();

        if (direction == Sort.Direction.ASC) {
            for (int year = startYear; year <= endYear; year++) {
                ranges.add(newShardRange(year, startDate, endDate));
            }
        } else {
            for (int year = endYear; year >= startYear; year--) {
                ranges.add(newShardRange(year, startDate, endDate));
            }
        }

        return ranges;
    }

    private ShardRange newShardRange(int year, LocalDate startDate, LocalDate endDate) {
        LocalDate shardStart = LocalDate.of(year, 1, 1);
        LocalDate shardEnd = LocalDate.of(year, 12, 31);

        LocalDate start = startDate.isAfter(shardStart) ? startDate : shardStart;
        LocalDate end = endDate.isBefore(shardEnd) ? endDate : shardEnd;

        return new ShardRange(year, start, end);
    }

    private Query buildTidRangeQuery(LocalDate startDate, LocalDate endDate) {
        String startKey = startDate.format(DATE_FORMAT);
        String endKey = endDate.format(DATE_FORMAT);
        // "~" is the highest printable ASCII, so it includes any ASCII suffix after yyyyMMdd.
        String endUpperBound = endKey + "~";

        return new Query(
                Criteria.where(TID_FIELD)
                        .gte(startKey)
                        .lte(endUpperBound)
        );
    }

    private String collectionName(int year) {
        return collectionPrefix + year;
    }

    private static final class ShardRange {
        private final int year;
        private final LocalDate startDate;
        private final LocalDate endDate;
        private long count;

        private ShardRange(int year, LocalDate startDate, LocalDate endDate) {
            this.year = year;
            this.startDate = startDate;
            this.endDate = endDate;
        }
    }

    public static final class PageResult<T> {
        private final List<T> items;
        private final long total;
        private final int pageNumber;
        private final int pageSize;

        public PageResult(List<T> items, long total, int pageNumber, int pageSize) {
            this.items = items;
            this.total = total;
            this.pageNumber = pageNumber;
            this.pageSize = pageSize;
        }

        public List<T> getItems() {
            return items;
        }

        public long getTotal() {
            return total;
        }

        public int getPageNumber() {
            return pageNumber;
        }

        public int getPageSize() {
            return pageSize;
        }

        public int getTotalPages() {
            if (pageSize == 0) {
                return 0;
            }
            return (int) Math.ceil((double) total / (double) pageSize);
        }
    }

    public static class Payment {
        private String tid;

        public String getTid() {
            return tid;
        }

        public void setTid(String tid) {
            this.tid = tid;
        }
    }
}
