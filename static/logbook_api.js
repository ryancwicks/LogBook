"use strict"

export {api_instance};

import { Entry } from "./logbook_entry.js";

function api_instance () {
    let requests = {
        add_item: "/api/add_item",
        get_all: "/api/all",
    }

    /**
     * Check that the required properties are sent from the server (success and message)
     * @param {object} json_object 
     * @return boolean
     */
    function hasRequiredProperties(json_object) {
        let state = json_object.hasOwnProperty("success");
        if (json_object.success === true) {
            return state;
        }
        state &= json_object.hasOwnProperty("message");
        return state;
    }

    /**
     * Grab the entire logbook from the server.
     * @return {array} table data.
     */
    async function getTable() {
        let request;
        let request_json;
        try {
            request = await fetch(requests.get_all);
            request_json = await request.json();
        } catch (e) {
            console.log(e);
            return;
        }

        if (!hasRequiredProperties(request_json)) {
            console.log("Returned message has incorrect fields.");
            return;
        }

        if (!request_json.success) {
            console.log(request_json.message);
            return;
        }

        let data = [];
        for (let i = 0; i < request_json.records.length; ++i) {
            let row = request_json.records[i];
            
            let new_entry = new Entry(
                row.id,
                row.log_time,
                row.body,);
            data.push(new_entry);
        }

        return data;
    };

    /**
     * Send data to the server to be added to the database/spreadsheet/drive
     * @param {object} data String to log.
     * @return {Entry} The logged entry data.
     */
    async function addEntry(data) {
        let request;
        let request_json;
        try {
            request = await fetch(requests.add_item, formRequest({log: data}));
            request_json = await request.json();
        } catch (e) {
            console.log(e);
            return;
        }

        if (!hasRequiredProperties(request_json)) {
            console.log("Returned message has incorrect fields.");
            return;
        }

        if (!request_json.success) {
            console.log(request_json.message);
            return;
        }

        if (!request_json.records.lenght != 1) {
            console.log("Invalid response, missing record object after database insertion.");
            return;
        }
        let row = request_json.records[0];
            
        let new_entry = new Entry(
            row.id,
            row.log_time,
            row.body,);

        return new_entry;

    };

    return {
        getTable: getTable,
        addEntry: addEntry,
    };

    /**
     * @brief Method to simplify creation of fetch POST requests
     * @param {object} data object 
     * @return {object} Returns a fetch post request object.
     */
    function formRequest (data_object, method="POST") {
        const fetch_request = {
            method: method, 
            mode: 'cors', 
            cache: 'no-cache', 
            credentials: 'same-origin', 
            headers: {
              'Content-Type': 'application/json',
            },
            redirect: 'follow', 
            referrerPolicy: 'no-referrer', 
            body: JSON.stringify(data_object) 
          };

        return fetch_request;
    }
}