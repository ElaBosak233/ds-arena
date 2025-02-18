import { AppBar, css, Toolbar, Typography } from "@mui/material";

export default function Navbar() {
    return (
        <AppBar
            position={"sticky"}
            color={"transparent"}
            variant={"outlined"}
            css={css`
                backdrop-filter: blur(8px);
                border-width: 0 0 1px 0;
            `}
        >
            <Toolbar>
                <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                    DS Arena
                </Typography>
            </Toolbar>
        </AppBar>
    );
}
