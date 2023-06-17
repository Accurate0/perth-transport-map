import { useAtom } from "jotai";
import { atom } from "jotai";
import { useEffect, useState } from "react";

const webSocket = atom<WebSocket>(
  new WebSocket(import.meta.env.VITE_WS_API_BASE ?? "")
);

const useWebSocket = (onMessage: (data: string) => void) => {
  const [socket] = useAtom(webSocket);
  const [isReady, setIsReady] = useState<boolean>();

  useEffect(() => {
    socket.onopen = () => {
      setIsReady(true);
    };

    socket.onmessage = (ev) => onMessage(ev.data);
  }, []);

  return { socket, isReady };
};

export default useWebSocket;
