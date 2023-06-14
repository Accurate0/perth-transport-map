import { getClient } from "@/lib/apollo-client";
import { gql } from "@apollo/client";

export default async function Home() {
  const { data } = await getClient().query({
    query: gql`
      {
        route(searchTerm: "arma") {
          routes {
            identifier
            timetableId
          }
        }
        liveTrips(timetableId: "PerthRestricted:RTG_10") {
          liveTrips
        }
      }
    `,
  });

  return <></>;
}
