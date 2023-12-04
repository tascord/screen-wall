## scw (screen wall)
Launch chromium (fullscreen) windows on multiple monitors. Useful for a screen wall.

### CLI flags
| Flag      | Description             | Example                                      |
| --------- | ----------------------- | -------------------------------------------- |
| -i <path> | Identify screen numbers | `scw.exe -i chrome.exe`                      |
| -u <url>  | Load config from URL    | `scw.exe -i https://example.com/config.json` |

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
            "url": "https://google.com"
        },
    ]
}
```