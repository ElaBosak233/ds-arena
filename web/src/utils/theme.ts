import { ThemeOptions } from "@mui/material";

export const themeOptions: ThemeOptions = {
    components: {
        MuiButton: {
            styleOverrides: {
                root: ({ theme }) => ({
                    borderRadius: "30px",
                    textTransform: "none",
                    fontWeight: "bold",
                    "&:has(>svg)": {
                        padding: "8px",
                        borderRadius: "50%",
                        minWidth: "1em",
                        minHeight: "1em",
                    },
                }),
            },
        },
    },
};
