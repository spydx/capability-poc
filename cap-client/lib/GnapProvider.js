import { createContext, useContext } from "react";
import useGnap from "./useGnap";

const GnapContext = createContext({
    user: null,
    redirect: false,
    redirectLogin: async () => {},
    tx: null,
    request: null,
    gnaprequest: null,
    setTransaction: async () => {},
    gnap_request: async () => {},
    gnap_contiuation: async () => {},
    showCreate: false,
    setShowCreate: async () => {},
    create_resourse: async () => {},
});

export default function GnapProvider({children}) {
    const gnap = useGnap();

    return (
        <GnapContext.Provider value={gnap}>{children}</GnapContext.Provider>
    )
}

export const useGnapContext = () => useContext(GnapContext);