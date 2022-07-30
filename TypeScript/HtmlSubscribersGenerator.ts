

class HtmlSubscribersGenerator {

    public static generateHtml(data: IInitializedStatus): string {

        return '<h3>Tables</h3>'
            + this.generateTablesHtml(data.tables);
    }



    private static generateTablesHtml(tables: ITableModel[]): string {
        let html = `<table class="table table-striped"><tr><th>Table</th><th>Persist</th><th>DataSize</th><th>Partitions</th><th>Records</th><th>Indexed Records</th><th>Last update</th></tr>`;

        let total_size = 0;
        let total_partitions = 0;
        let total_records = 0;
        let total_indexed_records = 0;
        for (let table of tables.sort((a, b) => a.name > b.name ? 1 : -1)) {

            let style = ' style="color:green" ';


            if (!table.lastPersistTime) {
                style = ' style="color:gray" ';
            }
            else
                if (table.lastPersistTime < table.lastUpdateTime) {
                    style = ' style="color:red" ';
                }

            let lastUpdateTime = new Date(table.lastUpdateTime / 1000);

            let lastPersistTime = "----";

            if (table.lastPersistTime) {
                lastPersistTime = new Date(table.lastPersistTime / 1000).toISOString();
            }


            let nextPersistTime = "---";

            if (table.nextPersistTime) {
                let as_time = new Date(table.nextPersistTime / 1000);
                nextPersistTime = as_time.toISOString();
            }



            html += '<tr><td>' + table.name + '</td><td>' + table.persistAmount + '</td><td>' + table.dataSize + '</td><td>' + table.partitionsCount + '</td><td>' + table.recordsAmount + '</td><td>' + table.expirationIndex + '</td>' +
                '<td' + style + '><div>UpdateTime: ' + lastUpdateTime.toISOString() + '</div><div>PersistTime: ' + lastPersistTime + '</div>' +
                '<div>NextPersist: ' + nextPersistTime + '</div>' + HtmlGraph.renderGraph(table.lastPersistDuration, v => Utils.format_duration(v), v => v, v => false) + '</td></tr>';

            total_size += table.dataSize;
            total_partitions += table.partitionsCount;
            total_records += table.recordsAmount;
            total_indexed_records += table.expirationIndex;

        }

        html += '<tr style="font-weight: bold; background-color:black; color:white;"><td>Total</td><td></td><td>DataSize: ' + total_size + '</td><td>Partitions: ' + total_partitions + '</td><td>Records: ' + total_records + '</td><td>Indexed records: ' + total_indexed_records + '</td>'
            + '<td></td></tr>';

        html += '</table>';

        return html;
    }




    private static renderName(name: string): string {
        let lines = name.split(';');

        let result = "";
        for (let line of lines) {
            result += "<div>" + line + "</div>";
        }

        return result;
    }


    private static renderTables(data: string[]): string {
        let result = "";

        for (let itm of data) {
            result += '<span class="badge badge-info" style="margin-left: 5px">' + itm + '</span>';
        }

        return result;

    }
}