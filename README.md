# Koda (.koda) - The Universal Data-Linking Language

Koda is a "Super-JSON" file format built in Rust that focuses on zero-dependency speed and extreme ease of use. It allows you to define data, link files, and execute scripts in a single, human-readable document.

---

## 🎓 Learning the Syntax

Koda is designed to be memorized in under 2 minutes. There are only 5 primary "Power Keys" you need to know.

### 1. Basic Key-Values
Unlike JSON, you don't need quotes for keys or strings. A simple colon separates them.
```
key: value
version: 1.0.2
status: online
```
### 2. Variables (@)
Variables allow you to store a value and reuse it anywhere. This makes bulk updates instant.

@brand: #FF5500
@api_v: v1

theme_color: @brand
login_url: https://api.com/@api_v/login

### 3. File Pointers (>)
This is where Koda connects your project. Use the > symbol to point to any other data file. Koda will automatically fetch that data.

# Imports the content of a JSON file into this key
user_database: >./data/users.json

# Imports raw text from a file
legal_footer: >./legal/disclaimer.txt

### 4. Logic & Execution (!)
If you need data that changes (like a timestamp or a git hash), use the ! symbol. Koda will run the command and save the result.

# Runs a shell command
last_update: !date

# Runs a custom script
api_token: !python3 ./scripts/get_token.py

### 5. Hierarchy (Indentation)
Koda uses indentation (like YAML) to organize data. You can nest things as deep as you want.

settings:
    network:
        port: 8080
        secure: true
    styling:
        color: @brand
        font: JetBrains Mono

---

## 🛠 Developer Integration

Because Koda is built in Rust as a cdylib, it can be imported into any language.

### JavaScript / Web
const data = koda.parse('name: Koda');
console.log(data.name);

### Python
config = koda.load("config.koda")
print(config["project_name"])

### C++ / C#
const char* json = koda_full_process("key: >file.json");

---

## 📝 Comparison Table

| Feature        | JSON | YAML | Koda    |
|----------------|------|------|---------|
| Comments       | ❌    | ✅    | ✅ (#)   |
| Variables      | ❌    | ⚠️    | ✅ (@)   |
| File Linking   | ❌    | ❌    | ✅ (>)   |
| Code Execution | ❌    | ❌    | ✅ (!)   |
| Syntax Weight  | Heavy| Med  | Ultra   |

---

## 🚀 Getting Started

1. Create a file named main.koda.
2. Define your links: Use > to point to your existing JSON/CSV files.
3. Compile: Use the Koda CLI to output a unified JSON for your app:
   koda build main.koda --output config.json

---

## 📜 License
MIT - Created with Rust. Zero Libraries. Zero Bloat.
