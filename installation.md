# 🦀 Rust Installation Guide (Beginner Friendly)

This guide helps you install **Rust** step by step, even if you are completely new to programming.

Rust works on **Windows**, **macOS**, and **Linux**.

---

## 📌 What is Rust?

Rust is a **fast**, **safe**, and **modern** programming language used for:
- System programming
- Backend development
- Command-line tools
- High-performance applications

---

## 🛠 What You Need Before Installing

- A computer (Windows / macOS / Linux)
- Internet connection
- Basic ability to use **Terminal / Command Prompt**

That’s it 👍

---

## 🚀 Step 1: Install Rust Using `rustup`

Rust is installed using a tool called **rustup**.  
`rustup` automatically manages Rust versions for you.

---

## 🪟 Windows Installation

### Step 1: Open Browser  
Go to:  
https://www.rust-lang.org/tools/install

### Step 2: Download Installer  
Click **“Download Rustup”**

### Step 3: Run the Installer  
- Double-click the downloaded file  
- A terminal window will open

### Step 4: Choose Default Installation  
When prompted:
Type **1** and press **Enter**

### Step 5: Wait for Installation  
Rust will install automatically.

### Step 6: Restart Your PC (Recommended)

---

## 🍎 macOS Installation

### Step 1: Open Terminal  
Press:

### Step 2: Run This Command
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Step 3: Choose Default Installation
when prompted:
type
```bash
1
```
### Step 4: Restart Terminal 

## 🐧 Linux Installation

### Step 1: Open Terminal
### Step 2: Run This Command
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Step 3: Select Default Installation

Type 1 and press Enter

### Step 4: Reload Environment
```bash
source $HOME/.cargo/env
```

✅ Step 2: Verify Installation

Open Terminal / Command Prompt and run:
```bash
rustc --version
```

Expected output:
```bash
rustc 1.xx.x (xxxx)
```
🎉 Rust is installed successfully!