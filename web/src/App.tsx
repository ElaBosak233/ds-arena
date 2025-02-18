import {
    Box,
    createTheme,
    css,
    ThemeProvider,
    CssBaseline,
    Stack,
} from "@mui/material";
import { themeOptions } from "@/utils/theme";
import Navbar from "@/components/Navbar";
import Submit from "@/components/Submit";
import Data from "@/components/Data";

function App() {
    const theme = createTheme(themeOptions);

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Box
                css={css`
                    display: flex;
                    flex-direction: column;
                    min-height: 100vh;
                `}
            >
                <Navbar />
                <Stack spacing={2} p={3} flex={1}>
                    <Submit />
                    <Data />
                </Stack>
            </Box>
        </ThemeProvider>
    );
}

export default App;
