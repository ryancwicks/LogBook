"use strict"

export { buildForm, buildTable };

import { api_instance } from './logbook_api.js';
import { Entry } from "./logbook_entry.js";

let table = document.createElement("TABLE");
let api = api_instance();

function buildForm (form_div_id) {
    let form_div = document.getElementById(form_div_id);

    if (!form_div) {
        console.log("Failed to load find form div.")
        return;
    }

    let text_area = document.createElement("textarea")
    text_area.rows=4;
    text_area.cols = 150;
    form_div.appendChild (text_area);
    let add_button = document.createElement("input");
    add_button.type = "button";
    add_button.value = "Submit";
    form_div.appendChild(add_button);
    text_area.focus();

    add_button.addEventListener("click", async function(){
        let text_data = text_area.value;
        if (text_data === "") {
            return;
        }

        let record = await api.addEntry(text_data);

        if (!record) {
            console.log("Did not recieve confirmation record from server.");
        }
        let next_element = table.firstChild.nextSibling;
        if (next_element) {
            table.insertBefore(record.tableRow(), next_element);
        } else {
            table.appendChild(record.tableRow());
        }

        text_area.value = "";
        text_area.focus();

    })


}

async function buildTable (table_div_id) {
    let table_div = document.getElementById(table_div_id);

    if (!table_div) {
        console.log("Failed to load find table div.")
        return;
    }

    table.id = "log_table";
    table_div.appendChild(table);

    //Header
    let table_header = document.createElement("TR");
    let time_header = document.createElement("TH");
    time_header.innerHTML = "<h2>Time (UTC) </h2>";
    time_header.width = 30
    let log_header = document.createElement("TH");
    log_header.innerHTML = "<h2>Log</h2>";
    log_header.width = "auto";
    table_header.appendChild(time_header);
    table_header.appendChild(log_header);
    table.appendChild(table_header);

    //get_data
    let data = await api.getTable();
    if (!data) {
        return;
    }

    for (let i = 0; i < data.length; ++i) {
        table.appendChild(data[i].tableRow());
    }

}