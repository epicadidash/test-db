# SQL DB 
## About
# Try runiing commands :
``
Under new updates
``

Sure! Below is a comprehensive and detailed SQL database data page structure with headers and descriptions that outline intrinsic values commonly found in a data page. I’ll format it to resemble what you might find in a real-world SQL database documentation.

---

### SQL Database Data Page Structure

**Database Name:** `CompanyDB`
**Table Name:** `EmployeeRecords`
**Page Type:** Data Page
**Page Number:** 1
**Page Size:** 8KB

---

#### 1. **Page Header**

- **Page Type:** Data Page
- **Page Number:** 1
- **Page Size:** 8192 bytes
- **Free Space:** 256 bytes
- **Row Count:** 5
- **Checksum:** 0xA1B2C3D4
- **Next Page Pointer:** Page 2
- **Previous Page Pointer:** Page 0

---

#### 2. **Metadata**

- **Table ID:** 1001
- **Last Modified:** 2024-10-01 10:15:00 UTC
- **Created On:** 2024-01-15 08:30:00 UTC
- **Index ID:** 2001 (Primary Key)
- **Transaction ID:** 3001 (Last Transaction ID)

---

#### 3. **Row Details**

| **Row ID** | **EmployeeID** | **FirstName** | **LastName** | **Email**               | **DepartmentID** | **HireDate**           | **Salary**  | **Status**   |
|------------|----------------|----------------|---------------|-------------------------|------------------|------------------------|-------------|--------------|
| 1          | 10001          | John           | Doe           | john.doe@example.com    | 10               | 2023-05-15 09:00:00 UTC| 60000.00    | Active       |
| 2          | 10002          | Jane           | Smith         | jane.smith@example.com   | 20               | 2022-03-20 09:00:00 UTC| 55000.00    | Active       |
| 3          | 10003          | Emily          | Johnson       | emily.johnson@example.com| 10               | 2021-11-10 09:00:00 UTC| 70000.00    | On Leave     |
| 4          | 10004          | Michael        | Brown         | michael.brown@example.com| 30               | 2020-06-05 09:00:00 UTC| 65000.00    | Active       |
| 5          | 10005          | Sarah          | Davis         | sarah.davis@example.com  | 20               | 2024-01-01 09:00:00 UTC| 72000.00    | Probation    |

---

#### 4. **Column Definitions**

- **EmployeeID** (INT): Unique identifier for each employee. Primary key.
- **FirstName** (VARCHAR(50)): First name of the employee.
- **LastName** (VARCHAR(50)): Last name of the employee.
- **Email** (VARCHAR(100)): Email address of the employee.
- **DepartmentID** (INT): Foreign key referencing the Departments table.
- **HireDate** (DATETIME): Date the employee was hired.
- **Salary** (DECIMAL(10,2)): Annual salary of the employee.
- **Status** (VARCHAR(20)): Employment status (e.g., Active, On Leave, Probation).

---

#### 5. **Constraints**

- **Primary Key Constraint:** EmployeeID is the primary key.
- **Foreign Key Constraint:** DepartmentID references DepartmentID in the Departments table.
- **Unique Constraint:** Email must be unique across records.

---

#### 6. **Indexes**

- **Index Name:** IDX_Employee_Email
  - **Index Type:** Non-clustered
  - **Columns Indexed:** Email
  - **Index Size:** 1 KB
  - **Row Count in Index:** 5

---

#### 7. **Transaction Log Information**

- **Last Transaction ID:** 3001
- **Transaction Type:** INSERT
- **Timestamp:** 2024-10-01 10:15:00 UTC
- **Affected Rows:** 1

---

#### 8. **Page Status Information**

- **Dirty Page:** Yes (changes not yet written to disk)
- **Checkpoint:** No
- **Last Flushed:** N/A (not applicable until dirty pages are flushed)

---

### End of Data Page

---

This structured format gives you a detailed overview of what a data page in a SQL database might look like, including headers that capture intrinsic values, data structure, and important metadata. Let me know if you need any specific modifications or additional details!
