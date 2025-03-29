In VS Code, you can search for JSON files containing a field named `quantity` with a floating-point value using **regular expressions** in the search feature. Here’s how:  

### Steps:  
1. **Open VS Code.**  
2. **Press `Ctrl + Shift + F`** (or `Cmd + Shift + F` on macOS) to open the global search.  
3. **Enable "Use Regular Expression"** (Click on the `.*` icon in the search bar).  
4. **Enter the following regex pattern in the search box:**  

   ```regex
   "quantity"\s*:\s*([0-9]*\.[0-9]+)
   ```

   - `"quantity"` → Matches the exact field name.  
   - `\s*:\s*` → Matches any spaces and the colon (`:`) after the key.  
   - `([0-9]*\.[0-9]+)` → Ensures the value is a floating-point number (e.g., `12.5`, `0.3`, `100.01`).  

5. **Make sure "Match Case" and "Match Whole Word" are disabled.**  
6. **Click on "Find"** to locate all JSON files with `quantity` as a floating-point value.  

If you also want to include negative numbers, use:  

```regex
"quantity"\s*:\s*-?[0-9]*\.[0-9]+
```

Let me know if you need further modifications!
