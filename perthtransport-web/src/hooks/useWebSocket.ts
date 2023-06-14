import { useEffect, useState } from "react";

const useWebSocket = (onMessage: (data: string) => void) => {
  const [socket, setSocket] = useState<WebSocket>();
  const [isReady, setIsReady] = useState<boolean>();

  useEffect(() => {
    const websocket = new WebSocket(import.meta.env.VITE_WS_API_BASE ?? "");

    setSocket(websocket);

    websocket.onopen = () => {
      setIsReady(true);
    };

    websocket.onmessage = (ev) => onMessage(ev.data);
  }, []);

  return { socket, isReady };
};

export default useWebSocket;
