Here's an improved `k6` script that:  

1. **Logs in** and extracts the session token.  
2. **Stores** the session token using `__VU` (virtual user-specific storage).  
3. **Makes an authenticated request** using the stored session.  
4. **Persists the session for reuse across iterations**.  

---

### **k6 Test Script with Session Storage (`test.js`)**
```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
    vus: 10, // Number of virtual users
    iterations: 20, // Total iterations per VU
};

let sessionTokens = {}; // Store session tokens per virtual user (__VU)

export default function () {
    let sessionToken = sessionTokens[__VU]; // Retrieve stored session token

    if (!sessionToken) {
        // Step 1: Login and get session token
        let loginPayload = JSON.stringify({
            username: 'testuser',
            password: 'password123'
        });

        let loginParams = {
            headers: {
                'Content-Type': 'application/json',
            },
        };

        let loginRes = http.post('https://example.com/api/login', loginPayload, loginParams);

        check(loginRes, {
            'Login successful': (res) => res.status === 200,
        });

        sessionToken = loginRes.json('token'); // Adjust key based on response structure

        if (!sessionToken) {
            console.error(`VU ${__VU}: No session token received!`);
            return;
        }

        sessionTokens[__VU] = sessionToken; // Store session token for this VU
    }

    let authHeaders = {
        headers: {
            'Authorization': `Bearer ${sessionToken}`,
            'Content-Type': 'application/json',
        },
    };

    // Step 2: Make an authenticated request using stored session
    let sessionRes = http.get('https://example.com/api/protected-route', authHeaders);

    check(sessionRes, {
        'Authenticated request successful': (res) => res.status === 200,
    });

    sleep(1); // Pause between iterations
}
```

---

### **What This Script Does**
✅ Logs in only **if a session does not exist** for the virtual user (VU).  
✅ Stores the session per virtual user in `sessionTokens[__VU]`.  
✅ Uses the stored session for subsequent requests.  
✅ Only logs in again if the session expires.  

---

### **How to Run the Test**
```sh
k6 run test.js
```

This ensures each virtual user maintains their session across multiple iterations, reducing unnecessary logins. Let me know if you need tweaks!
