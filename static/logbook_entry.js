"use strict"

export {Entry};

function Entry(id, time, data) {
    if (this instanceof Entry === false) {
        return new Entry(id, time, data);
    }
    this.id = id;
    this.time = time;
    this.data = data;
}

Entry.prototype.tableRow = function () {
    let table_row = document.createElement("TR");
    table_row.id = String(this.id);

    let time = document.createElement("td");
    time.innerHTML = this.time;
    
    let log_cell = document.createElement("td");
    let log = document.createElement("a");
    log.innerHTML = this.data;
    log_cell.appendChild(log);

    table_row.appendChild(time);
    table_row.appendChild(log_cell);

    return table_row;
}