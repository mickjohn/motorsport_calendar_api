function send_delete_request(href, item_type) {
  if (window.confirm("Are you sure you want to delete this " + item_type + "?")) {
    $.ajax({
      async: false,
      url: href,
      type: 'DELETE',
      success: function(result) { location.reload() }
    });
  }
}

$(function() {
	$(".delete_session").on("click",function(e) {
		e.preventDefault(); // cancel the link itself
    send_delete_request(this.href, "session");
	});
});

$(function() {
	$(".delete_event").on("click",function(e) {
		e.preventDefault(); // cancel the link itself
    send_delete_request(this.href, 'event');
	});
});
