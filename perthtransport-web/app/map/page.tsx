import useRouteQuery from "@/queries/useRouteQuery";
import Map from "@/components/Map";

export default async function Page() {
  // const { data } = await useRouteQuery("arma");

  return (
    <>
      <Map />
    </>
  );
}
