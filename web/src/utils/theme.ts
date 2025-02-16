import { ThemeOptions } from "@mui/material";

export const themeOptions: ThemeOptions = {
    palette: {
        // primary: {
        //     main: "#6750a4",
        // },
        // secondary: {
        //     main: "#625b71",
        // },
    },
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
