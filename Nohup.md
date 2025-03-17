### **What is `nohup`?**  
`nohup` (short for **"no hang up"**) is a Unix command that allows you to run a command in the background even after logging out or closing the terminal. It prevents the process from being terminated when the terminal session ends.

---

### **How `nohup` Works**  
When a process runs in a terminal, it is linked to the terminal session. If the session is closed, the process receives a `SIGHUP` (Hangup) signal and terminates. `nohup` prevents this by ignoring `SIGHUP`, allowing the process to continue running in the background.

---

### **Basic Usage**  
```sh
nohup command &
```
- `nohup` ensures the process doesn’t receive a `SIGHUP` signal.
- The `&` at the end runs the process in the background.
- By default, output is redirected to `nohup.out` in the current directory.

Example:  
```sh
nohup python3 my_server.py &
```
This runs `my_server.py` in the background and allows you to close the terminal.

---

### **Redirecting Output**  
By default, `nohup` writes output to `nohup.out`. You can specify your own log file:  
```sh
nohup my_command > my_log.log 2>&1 &
```
- `>` redirects standard output to `my_log.log`.  
- `2>&1` ensures error messages are also written to the same log file.  

Example for a Rust server:  
```sh
nohup cargo run --release > server.log 2>&1 &
```

---

### **Disowning the Process (Optional)**  
Even with `nohup`, the process may still be tied to the shell session in some cases. To completely detach it, use:  
```sh
disown -h %1
```
This ensures the process is not terminated if you log out.

---

### **Checking Running Processes**  
To check if your process is running:  
```sh
ps aux | grep my_command
```
Or use `pgrep`:  
```sh
pgrep -fl my_command
```

---

### **Killing the Process (If Needed)**  
To stop the process:  
```sh
pkill -f my_command
```
Or find its PID and kill it manually:  
```sh
ps aux | grep my_command
kill <PID>
```

---

### **Alternatives to `nohup`**  
- **`tmux` or `zellij`** (better for managing multiple sessions)  
- **`screen`** (similar to `tmux`)  
- **`systemd` service** (for persistent background services)  

Would you like to set up a `systemd` service instead for automatic restarts?
