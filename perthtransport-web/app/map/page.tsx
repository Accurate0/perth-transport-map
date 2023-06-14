"use client";
import useRouteQuery from "@/queries/useRouteQuery";
import Map from "@/components/Map";
import SearchBox from "@/components/SearchBox";
import useWebSocket from "@/lib/useWebSocket";
import { useState } from "react";

export default async function Page() {
  // const { data } = await useRouteQuery("arma");

  return (
    <>
      <SearchBox />
      <Map />
    </>
  );
}
