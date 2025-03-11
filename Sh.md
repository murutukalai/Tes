If you want an open-source API client that runs on Linux **without requiring `sudo` installation**, your best options are:  

### 1. **Bruno**  
   - Open-source, Thunder Client alternative.  
   - No `sudo` required, runs as a portable binary.  
   - **Installation:**  
     ```bash
     curl -L https://github.com/usebruno/bruno/releases/latest/download/bruno-linux-x64.tar.gz -o bruno.tar.gz
     tar -xvzf bruno.tar.gz
     ./bruno
     ```  
   - [GitHub Repo](https://github.com/usebruno/bruno)  

### 2. **Hoppscotch** (Web-based, No Installation)  
   - Works **entirely in the browser**, no installation needed.  
   - **Run it directly:** [hoppscotch.io](https://hoppscotch.io)  

### 3. **Insomnia** (Portable AppImage)  
   - Download and run **without `sudo`**.  
   - **Installation:**  
     ```bash
     wget https://github.com/Kong/insomnia/releases/latest/download/Insomnia.AppImage
     chmod +x Insomnia.AppImage
     ./Insomnia.AppImage
     ```  
   - [GitHub Repo](https://github.com/Kong/insomnia)  

These options will let you run an API testing client on Linux without needing root privileges. Let me know if you need more details!
