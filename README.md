## scw (screen wall)
Launch chromium (full-screen) windows on multiple monitors. Useful for a screen wall.

### CLI flags
| Flag      | Description                        | Example                                      |
| --------- | ---------------------------------- | -------------------------------------------- |
| -i <path> | Identify screen numbers            | `scw.exe -i chrome.exe`                      |
| -u <url>  | Load config from URL               | `scw.exe -i https://example.com/config.json` |
| -p        | Persist profiles                   | `scw.exe -p`                                 |
| -f        | Use full-screen, rather than kiosk | `scw.exe -f`                                 |

### config.json (see example)[/config.json]
```jsonc
{
    // Path to chromium browser
    "chrome_path": "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
    "windows": [
        {
            // Screen number
            "screen": 0,

            // Webpage URL to display
            "url": "https://google.com" // Add comma separated values to split the window
        },
    ]
}
```