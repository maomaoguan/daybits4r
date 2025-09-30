(function ($) {
  'use strict';

  function createRipple($target, event) {
    var offset = $target.offset();
    var x = event.pageX - offset.left;
    var y = event.pageY - offset.top;
    var $r = $('<span/>', { class: 'ripple' });
    $r.css({ left: x + 'px', top: y + 'px' });
    $target.append($r);
    setTimeout(function () { $r.remove(); }, 650);
  }

  function upsertUsedChip(key, label) {
    var $wrap = $('.used-filters');
    var safeKey = String(key);
    var chipId = 'used-' + safeKey;
    var $existing = $('#' + chipId);
    if ($existing.length) {
      $existing.find('.label').text(label);
      return $existing;
    }
    var $chip = $('<span/>', { id: chipId, class: 'used-chip', 'data-key': safeKey });
    var $label = $('<span/>', { class: 'label', text: label });
    var $remove = $('<button/>', { type: 'button', class: 'remove', 'aria-label': 'Remove ' + label });
    $remove.append('<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M19 6.41 17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/></svg>');
    $chip.append($label, $remove);
    $wrap.append($chip);
    return $chip;
  }

  function removeUsedChip(key) {
    $('#used-' + key).remove();
  }

  function currentState() {
    return {
      query: $('#globalSearch').val().trim(),
      category: $('#filterCategory').val() || '',
      price: $('#filterPrice').val() || '',
      toggles: $('.chip[aria-pressed="true"]').map(function () { return $(this).data('filter'); }).get()
    };
  }

  function announceState() {
    var s = currentState();
    var msg = 'Searching for "' + (s.query || 'all') + '"'
      + (s.category ? (', in ' + s.category) : '')
      + (s.price ? (', price ' + s.price) : '')
      + (s.toggles.length ? (', filters: ' + s.toggles.join(', ')) : '');
    // For demo, log to console. In a real app, trigger the search request here.
    console.log('[Search]', msg);
  }

  $(function () {
    var $search = $('#globalSearch');
    var $clear = $('.search-input__clear');

    function updateClearVisibility() {
      var hasValue = $search.val().length > 0;
      $clear.prop('hidden', !hasValue);
    }

    $search.on('input', updateClearVisibility);
    updateClearVisibility();

    $clear.on('click', function () {
      $search.val('');
      updateClearVisibility();
      $search.trigger('focus');
    });

    $('#submitSearch').on('click', function () {
      announceState();
    });

    $search.on('keydown', function (e) {
      if (e.key === 'Enter') {
        announceState();
      }
    });

    $('.chip').on('click', function (e) {
      var $chip = $(this);
      createRipple($chip, e);
      var isActive = $chip.attr('aria-pressed') === 'true';
      $chip.attr('aria-pressed', String(!isActive));
      $chip.toggleClass('is-active', !isActive);
      var key = String($chip.data('filter'));
      var labelText = $chip.text().trim();
      if (!isActive) {
        upsertUsedChip('chip-' + key, labelText);
      } else {
        removeUsedChip('chip-' + key);
      }
      announceState();
    });

    $('#filterCategory').on('change', function () {
      var value = $(this).val();
      if (value) {
        upsertUsedChip('category', 'Category: ' + value);
      } else {
        removeUsedChip('category');
      }
      announceState();
    });

    $('#filterPrice').on('change', function () {
      var value = $(this).val();
      if (value) {
        var text = $('#filterPrice option:selected').text();
        upsertUsedChip('price', 'Price: ' + text);
      } else {
        removeUsedChip('price');
      }
      announceState();
    });

    $('.used-filters').on('click', '.remove', function () {
      var $chip = $(this).closest('.used-chip');
      var key = $chip.data('key');
      if (key === 'category') { $('#filterCategory').val(''); }
      if (key === 'price') { $('#filterPrice').val(''); }
      $chip.remove();
      announceState();
    });

    $('#btnReset').on('click', function () {
      $('#filterCategory').val('');
      $('#filterPrice').val('');
      $('.chip').attr('aria-pressed', 'false').removeClass('is-active');
      $('.used-filters').empty();
      $('#globalSearch').val('');
      updateClearVisibility();
      announceState();
    });

    $('#btnApply').on('click', function () {
      // In a real application, submit the form or trigger a fetch here.
      // We just log the state to demonstrate.
      announceState();
      // Provide a tiny visual feedback
      var $btn = $(this);
      $btn.addClass('is-bouncing');
      setTimeout(function(){ $btn.removeClass('is-bouncing'); }, 300);
    });
  });
})(jQuery);

