import { useAtom } from "jotai";
import { atom } from "jotai";
import { useEffect, useState } from "react";

const webSocket = atom<WebSocket>(new WebSocket(import.meta.env.VITE_WS ?? ""));

const useWebSocket = (onMessage: (data: string) => void) => {
  const [socket] = useAtom(webSocket);
  const [isReady, setIsReady] = useState<boolean>();

  useEffect(() => {
    socket.onopen = () => {
      setIsReady(true);
    };

    socket.onmessage = (ev) => onMessage(ev.data);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return { socket, isReady };
};

export default useWebSocket;
