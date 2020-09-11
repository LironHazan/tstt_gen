const fs = require('fs');
const readline = require('readline');
const {google} = require('googleapis');

const SCOPES = ['https://www.googleapis.com/auth/spreadsheets.readonly'];
const TOKEN_PATH = 'private/token.json';

// Load client secrets from a local file.
fs.readFile('private/credentials.json', (err, content) => {
    if (err) return console.log('Error loading client secret file:', err);
    // Authorize a client with credentials, then call the Google Sheets API.
    authorize(JSON.parse(content), fetchTables);
});

/**
 * Create an OAuth2 client with the given credentials, and then execute the
 * given callback function.
 * @param {Object} credentials The authorization client credentials.
 * @param {function} callback The callback to call with the authorized client.
 */
function authorize(credentials, callback) {
    const {client_secret, client_id, redirect_uris} = credentials.installed;
    const oAuth2Client = new google.auth.OAuth2(client_id, client_secret, redirect_uris[0]);

    // Check if we have previously stored a token.
    fs.readFile(TOKEN_PATH, (err, token) => {
        if (err) return getNewToken(oAuth2Client, callback);
        oAuth2Client.setCredentials(JSON.parse(token));
        callback(oAuth2Client);
    });
}

/**
 * Get and store new token after prompting for user authorization, and then
 * execute the given callback with the authorized OAuth2 client.
 * @param {google.auth.OAuth2} oAuth2Client The OAuth2 client to get token for.
 * @param {getEventsCallback} callback The callback for the authorized client.
 */
function getNewToken(oAuth2Client, callback) {
    const authUrl = oAuth2Client.generateAuthUrl({
        access_type: 'offline',
        scope: SCOPES,
    });
    console.log('Authorize this app by visiting this url:', authUrl);
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });
    rl.question('Enter the code from that page here: ', (code) => {
        rl.close();
        oAuth2Client.getToken(code, (err, token) => {
            if (err) return console.error('Error while trying to retrieve access token', err);
            oAuth2Client.setCredentials(token);
            // Store the token to disk for later program executions
            fs.writeFile(TOKEN_PATH, JSON.stringify(token), (err) => {
                if (err) return console.error(err);
                console.log('Token stored to', TOKEN_PATH);
            });
            callback(oAuth2Client);
        });
    });
}

/**
 * Prints the names and majors of students in a sample spreadsheet:
 * @see https://docs.google.com/spreadsheets/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgvE2upms/edit
 * @param {google.auth.OAuth2} auth The authenticated Google OAuth client.
 */
function fetchTables(auth) {
    const sheets = google.sheets({version: 'v4', auth});
    fs.readFile('private/table_meta.json', (err, data) => {
        if (err) {
            return console.log('error reading private/table_meta.json');
        }
       const { tables, id } = JSON.parse(data);
        const procedures = [];

        for (const table of tables) {
            console.log('pushing ', table);
            procedures.push(fetchTable(sheets, table, id))
        }

        Promise.all(procedures)
            .then(() => console.log('done'))
            .catch((err) => console.log(err))
            .finally(() => console.log('bye bye'))
    });
}

/**
 *
 * @param sheets = the google sheets api object
 * @param range = table named range {string}
 * @param spreadsheetId {string}
 * @returns {Promise<void>}
 */
function fetchTable(sheets, range, spreadsheetId) {
    return new Promise((resolve, reject) => {
        sheets.spreadsheets.values.get({spreadsheetId, range },
            (err, res) => onTableFetched(err, res, range) );

        function onTableFetched(err, res, range) {
            if (err) {
                console.log('The API returned an error: ' + err);
                return reject();
            }
            const rows = res.data.values;
            return tableAsJson(rows, range);
        }
    });
}

/**
 *
 * @param rows
 * @param tableName --> named range
 * @returns {Promise<void>}
 */
function tableAsJson(rows, tableName) {
    return new Promise((resolve, reject) => {
        if (rows) {
            fs.writeFile(`tables/${tableName}.json`, JSON.stringify(rows), (err) => {
                if (err) {
                    console.error(err);
                    reject(err);
                } else {
                    resolve();
                }
            });
        } else {
            console.log('No data found.');
            resolve();
        }
    })
}