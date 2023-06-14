import { useEffect } from "react";

const useWebSocket = (onMessage: (data: string) => void) => {
  useEffect(() => {
    const websocket = new WebSocket(import.meta.env.VITE_WS_API_BASE ?? "");

    websocket.onopen = () => {
      websocket.send(
        JSON.stringify({
          tripId: "PerthRestricted:3458747",
        })
      );
    };

    websocket.onmessage = (ev) => onMessage(ev.data);
  }, []);
};

export default useWebSocket;
