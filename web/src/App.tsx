import { Button, createTheme, ThemeProvider } from "@mui/material";
import { useState } from "react";
import { themeOptions } from "./utils/theme";

function App() {
    const [count, setCount] = useState(0);

    const theme = createTheme(themeOptions);

    return (
        <ThemeProvider theme={theme}>
            <div
                style={{
                    display: "flex",
                    gap: "10px",
                }}
            >
                <Button variant={"contained"}>Hello</Button>
                <Button variant={"outlined"}>Hello</Button>
                <Button variant={"text"}>Hello</Button>
            </div>
        </ThemeProvider>
    );
}

export default App;
