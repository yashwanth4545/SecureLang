import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    console.log('SecureLang extension is now active!');

    // Hover Provider
    const hoverProvider = vscode.languages.registerHoverProvider('securelang', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);
            
            if (word === 'user' || word === 'secure' || word === 'authenticate') {
                return new vscode.Hover(`**${word}**: SecureLang built-in security DSL keyword.`);
            }
            return new vscode.Hover(`Type information for: ${word}`);
        }
    });

    // Diagnostics Mock (LSP foundation)
    const diagnosticCollection = vscode.languages.createDiagnosticCollection('securelang');
    
    // Commands
    let disposable = vscode.commands.registerCommand('securelang.compile', () => {
        vscode.window.showInformationMessage('Compiling SecureLang project...');
        // In a real LSP, this would trigger the Rust engine and pipe diagnostics back to diagnosticCollection
    });

    context.subscriptions.push(hoverProvider, diagnosticCollection, disposable);
}

export function deactivate() {}
