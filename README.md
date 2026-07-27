<div align="center">
  <h1>🛡️ SecureLang</h1>
  <p><strong>The Ultimate Security-Focused Programming Environment.</strong></p>
</div>

---

## 📖 What is SecureLang?

**SecureLang** is an enterprise-grade programming language and ecosystem built specifically for security professionals and developers who demand memory safety, strict execution controls, and unbreakable code. 

You can safely use SecureLang to write, analyze, and deploy highly secure applications.

## 🚀 How to Use SecureLang

This repository provides the public tools you need to write and run SecureLang code.

### 1. Install the VS Code Extension
For the best writing experience, install the official SecureLang VS Code extension.
1. Open Visual Studio Code.
2. Go to the Extensions panel (`Ctrl+Shift+X` or `Cmd+Shift+X`).
3. Search for **SecureLang** and click **Install**.
*(Note: If you have the `.vsix` file from our releases, you can install it manually by dragging it into the extensions panel).*

### 2. Download the Compiler CLI
To run your SecureLang code, you need the engine:
1. Head to the **Releases** tab on this GitHub repository.
2. Download the latest `securelang-cli` executable for your operating system (Windows, macOS, or Linux).
3. Add the executable to your system's `PATH`.

### 3. Run Your First Secure File
Create a new file called `main.sl` and write your SecureLang code:

```securelang
fn main() {
    print("Executing secure environment...");
}
```

Then, compile and run it securely from your terminal:
```bash
securelang-cli run main.sl
```

## ⚖️ License and Usage Restrictions

SecureLang is provided exclusively for your personal and internal business security purposes. 

**Important Restrictions:**
- You **may use** this software to secure and run your projects.
- You **may NOT** extract, modify, redesign, decompile, or redistribute the source code or any part of this project.

For the full legal terms, please read the [LICENSE](./LICENSE) file carefully before using the software. By using SecureLang, you agree to those terms.
