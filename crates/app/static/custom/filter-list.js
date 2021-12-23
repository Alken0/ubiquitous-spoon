function filter_list(self, list_name) {
	var value = self.value.toLowerCase();

	$(`#${list_name} li`).filter(function () {
		$(this).toggle($(this).text().toLowerCase().indexOf(value) > -1)
	});
}
