For bulk transfers from **ICICI Bank to another bank**, ICICI typically provides file formats like **Excel (.xls/.xlsx), CSV (.csv), or text (.txt) files**. The specific format depends on **ICICI’s Corporate Internet Banking (CIB) system** or any **ERP integration**.

### **ICICI Bank Bulk Transfer File Format (.txt)**
Below is a sample text file format for **bulk NEFT/RTGS/IMPS transfer**:

```
TransactionType|BeneficiaryName|BeneficiaryAccount|IFSC|Amount|Remarks|CustomerReferenceNo
NEFT|John Doe|123456789012|ICIC0001234|5000.00|Invoice Payment|INV001
RTGS|Jane Smith|987654321012|HDFC0005678|100000.00|Vendor Payment|VND002
IMPS|Rahul Kumar|654321098765|SBIN0003456|1500.00|Salary Payment|SAL003
```

---

### **Explanation of Each Field**
1. **TransactionType** – Type of transfer (`NEFT`, `RTGS`, or `IMPS`).
2. **BeneficiaryName** – Name of the receiver.
3. **BeneficiaryAccount** – Receiver’s bank account number.
4. **IFSC** – Receiver’s **bank branch IFSC code**.
5. **Amount** – Transfer amount in INR.
6. **Remarks** – Purpose of the transaction.
7. **CustomerReferenceNo** – Unique reference for tracking.

---

### **How to Use This File**
1. **Format the file as per ICICI’s requirements**.
2. **Upload to ICICI Bank’s Corporate Internet Banking** under the bulk transfer section.
3. **Validate the details** before proceeding.
4. **Authorize the transactions** for processing.

Let me know if you need a sample file in `.txt` or `.csv` format!


______

Here’s a sample **ICICI Bank bulk transfer text file** along with an explanation of the format.

---

### **Sample Bulk Transfer File (TXT Format)**  
```
NEFT|Amit Sharma|123456789012|ICIC0001234|5000.00|Invoice Payment|INV001
RTGS|Priya Verma|987654321098|HDFC0005678|150000.00|Vendor Payment|VND002
IMPS|Rohan Mehta|654321098765|SBIN0003456|2500.00|Salary Payment|SAL003
```

---

### **Explanation of Each Field**
| Field Name            | Description |
|----------------------|-------------|
| **TransactionType**   | Type of transfer: `NEFT`, `RTGS`, or `IMPS` |
| **BeneficiaryName**   | Name of the receiver |
| **BeneficiaryAccount** | Receiver’s bank account number |
| **IFSC**              | IFSC code of the receiver’s bank branch |
| **Amount**            | Transfer amount in INR |
| **Remarks**           | Reason for the transaction (e.g., `Invoice Payment`, `Vendor Payment`, `Salary Payment`) |
| **CustomerReferenceNo** | Unique reference for tracking |

---

### **Steps to Use This File**
1. **Prepare the file**: Enter multiple transactions in the same format.
2. **Save as `.txt` file**: Use a text editor like **Notepad** and save the file with **UTF-8 encoding**.
3. **Upload to ICICI Bank’s Corporate Internet Banking (CIB)** under **Bulk Transfer**.
4. **Validate the details**: ICICI Bank will check the format and details.
5. **Authorize the payments**: An approver in your account must approve transactions before processing.

---

### **Need a CSV File?**
If you prefer a **CSV file**, the format would be:
```
TransactionType,BeneficiaryName,BeneficiaryAccount,IFSC,Amount,Remarks,CustomerReferenceNo
NEFT,Amit Sharma,123456789012,ICIC0001234,5000.00,Invoice Payment,INV001
RTGS,Priya Verma,987654321098,HDFC0005678,150000.00,Vendor Payment,VND002
IMPS,Rohan Mehta,654321098765,SBIN0003456,2500.00,Salary Payment,SAL003
```

Would you like me to generate and send the actual `.txt` or `.csv` file for download?
