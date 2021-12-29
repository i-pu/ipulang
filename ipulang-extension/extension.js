"use strict";
const path = require('path');
const vscode = require("vscode");
const languageclient = require("vscode-languageclient");

let client;

function activate(context) {
    try {
        vscode.window.showInformationMessage(`Extension 'vscode-language-server' is now active.`);

        const serverOptions = {
            command: "/home/kafuhamada/Documents/github.com/i-pu/ipulang/target/debug/ipulang-lsp",
            args: [],
        };
        const clientOptions = {
            documentSelector: [
                {
                    scheme: "file",
                    language: "ipulang",
                }
            ],
        };

        client = new languageclient.LanguageClient("ipulang-lsp", serverOptions, clientOptions)
        
        context.subscriptions.push(vscode.commands.registerCommand('ipulang-extension.restartServer', async () => {
            vscode.window.showInformationMessage(`restart server`);
            await client.stop();
            client = new languageclient.LanguageClient("ipulang-lsp", serverOptions)
            client.start();
        }))

        client.start();
    } catch (e) {
        vscode.window.showErrorMessage("ipulang-server couldn't be started.");
    }
}

function deactivate() {
    if (client) return client.stop();
}

module.exports = { activate, deactivate }