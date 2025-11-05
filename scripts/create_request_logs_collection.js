// Usage:
//   mongosh "mongodb://localhost:27017" scripts/create_request_logs_collection.js
//
// The script creates a collection named `requestLogs` in the `appdb` database
// and configures a TTL index on the `requestTime` field so that documents are
// automatically removed 5 days (432000 seconds) after their request timestamp.

const databaseName = 'appdb';
const collectionName = 'requestLogs';
const ttlInSeconds = 5 * 24 * 60 * 60; // 5 days

const dbHandle = db.getSiblingDB(databaseName);

// Create the collection if it does not already exist and enforce that
// `requestTime` is stored as an ISODate (MongoDB `Date`) so the TTL index works.
// MongoDB TTL indexes operate on `Date` values; using this type satisfies the
// requirement for a timestamp column.
dbHandle.createCollection(collectionName, {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['requestTime'],
      properties: {
        requestTime: {
          bsonType: 'date',
          description: 'Request timestamp; documents expire 5 days after this time.'
        }
      }
    }
  },
  validationLevel: 'strict',
  validationAction: 'error'
});

// Create (or replace) the TTL index. If it already exists, this call will no-op.
dbHandle[collectionName].createIndex(
  { requestTime: 1 },
  {
    expireAfterSeconds: ttlInSeconds,
    name: 'requestTime_ttl_5d'
  }
);

// Optional: insert a sample document when the collection is empty so you can
// verify the schema and TTL behaviour quickly.
if (dbHandle[collectionName].countDocuments({}) === 0) {
  dbHandle[collectionName].insertOne({
    requestTime: new Date(),
    payload: 'example'
  });
}

print(`Collection '${collectionName}' in database '${databaseName}' is ready with a 5-day TTL on 'requestTime'.`);
