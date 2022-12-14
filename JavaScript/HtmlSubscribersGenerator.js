var HtmlSubscribersGenerator = /** @class */ (function () {
    function HtmlSubscribersGenerator() {
    }
    HtmlSubscribersGenerator.generateHtml = function (data) {
        return '<h3>Tables</h3>'
            + this.generateTablesHtml(data.tables);
    };
    HtmlSubscribersGenerator.generateTablesHtml = function (tables) {
        var html = "<table class=\"table table-striped\"><tr><th>Table</th><th>Persist</th><th>DataSize</th><th>Partitions</th><th>Records</th><th>Indexed Records</th><th>Last update</th></tr>";
        var total_size = 0;
        var total_partitions = 0;
        var total_records = 0;
        var total_indexed_records = 0;
        for (var _i = 0, _a = tables.sort(function (a, b) { return a.name > b.name ? 1 : -1; }); _i < _a.length; _i++) {
            var table = _a[_i];
            var style = ' style="color:green" ';
            if (!table.lastPersistTime) {
                style = ' style="color:gray" ';
            }
            else if (table.lastPersistTime < table.lastUpdateTime) {
                style = ' style="color:red" ';
            }
            var lastUpdateTime = new Date(table.lastUpdateTime / 1000);
            var lastPersistTime = "----";
            if (table.lastPersistTime) {
                lastPersistTime = new Date(table.lastPersistTime / 1000).toISOString();
            }
            var nextPersistTime = "---";
            if (table.nextPersistTime) {
                var as_time = new Date(table.nextPersistTime / 1000);
                nextPersistTime = as_time.toISOString();
            }
            html += '<tr><td>' + table.name + '</td><td>' + table.persistAmount + '</td><td>' + table.dataSize + '</td><td>' + table.partitionsCount + '</td><td>' + table.recordsAmount + '</td><td>' + table.expirationIndex + '</td>' +
                '<td' + style + '><div>UpdateTime: ' + lastUpdateTime.toISOString() + '</div><div>PersistTime: ' + lastPersistTime + '</div>' +
                '<div>NextPersist: ' + nextPersistTime + '</div>' + HtmlGraph.renderGraph(table.lastPersistDuration, function (v) { return Utils.format_duration(v); }, function (v) { return v; }, function (v) { return false; }) + '</td></tr>';
            total_size += table.dataSize;
            total_partitions += table.partitionsCount;
            total_records += table.recordsAmount;
            total_indexed_records += table.expirationIndex;
        }
        html += '<tr style="font-weight: bold; background-color:black; color:white;"><td>Total</td><td></td><td>DataSize: ' + total_size + '</td><td>Partitions: ' + total_partitions + '</td><td>Records: ' + total_records + '</td><td>Indexed records: ' + total_indexed_records + '</td>'
            + '<td></td></tr>';
        html += '</table>';
        return html;
    };
    HtmlSubscribersGenerator.renderName = function (name) {
        var lines = name.split(';');
        var result = "";
        for (var _i = 0, lines_1 = lines; _i < lines_1.length; _i++) {
            var line = lines_1[_i];
            result += "<div>" + line + "</div>";
        }
        return result;
    };
    HtmlSubscribersGenerator.renderTables = function (data) {
        var result = "";
        for (var _i = 0, data_1 = data; _i < data_1.length; _i++) {
            var itm = data_1[_i];
            result += '<span class="badge badge-info" style="margin-left: 5px">' + itm + '</span>';
        }
        return result;
    };
    return HtmlSubscribersGenerator;
}());
//# sourceMappingURL=HtmlSubscribersGenerator.js.map