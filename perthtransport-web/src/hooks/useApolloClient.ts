import { ApolloClient, InMemoryCache } from "@apollo/client";
import { useMemo } from "react";

const useApolloClient = () => {
  return useMemo(() => {
    return new ApolloClient({
      uri: import.meta.env.VITE_GRAPHQL_API_BASE,
      cache: new InMemoryCache(),
    });
  }, []);
};

export default useApolloClient;
