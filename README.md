# URL Shortening Service
This is a lightweight URL shortener utilizing Actix Web framework for asynchronous request handling
and SQLx for efficient concurrent database management. It implements a RESTful API for creating URLs and
redirecting URL requests and utilizes a custom error-handling module for increased data integrity and server
stability.

## Required Installations
### Rust
Utilize the Rust toolchain installer ```rustup``` to install Rust.  
Rust official site: https://www.rust-lang.org/

### sqlx-cli
This command-line tool assists with database management and migration.  
```cargo install sqlx-cli --no-default-features --features "sqlite, native-tls"```

## Running the Application
The below commands are formatted for PowerShell.

### Navigate to Project
```cd \project\path```

### Build Project (powershell)
This command sets the database as an environment variable and builds the project.  
```$env:DATABASE_URL="sqlite:url_mappings.db"; cargo build```

### Run Project
```cargo run```

The server is now running locally on ```http://127.0.0.1:8080```  
Keep the terminal window open to keep the server up.

## API Usage
While the server is running, the API can be utilized in a new PowerShell terminal.

### To Shorten a URL
This command sends a ```POST``` request to the ```/shorten``` endpoint  
```Invoke-WebRequest -Uri http://127.0.0.1:8080/shorten -Method POST -Body '{"original_url": "{original url}"}' -ContentType "application/json"```

The server will respond with a JSON object holding the new URL.  
ex: ```{"short_url":"http://localhost:8080/abc123"}```

### To Request Original URL from Shortened URL
```$response = Invoke-WebRequest -Uri {short address} -Method GET -MaximumRedirection 0 -ErrorAction SilentlyContinue $response.Headers.Location```

### To Utilize Shortened URL
Enter short address (ex: ```http://localhost:8080/abc123```) in web browser address bar while the local server is running.
