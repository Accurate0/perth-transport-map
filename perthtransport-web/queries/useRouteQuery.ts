import { getClient } from "@/lib/apollo-client";
import { gql, useQuery } from "@apollo/client";

const GET_ROUTES = gql`
  query GetRoutes($searchTerm: String!) {
    route(searchTerm: $searchTerm) {
      routes {
        identifier
        timetableId
      }
    }
  }
`;

interface RouteQueryResponse {
  route: {
    routes: [
      {
        identifier: string;
        timetableId: string;
      }
    ];
  };
}

const useRouteQuery = async (searchTerm: string) => {
  return await getClient().query<RouteQueryResponse>({
    query: GET_ROUTES,
    variables: {
      searchTerm,
    },
  });
};

export default useRouteQuery;
