import { useEffect, useState } from "react";

const useHealthCheck = () => {
  const [isHealthy, setIsHealthy] = useState(false);

  useEffect(() => {
    const healthCheck = async () => {
      const response = await fetch(
        `${import.meta.env.VITE_API_BASE ?? ""}/status/health`
      );

      setIsHealthy(response.status == 204);
    };

    healthCheck();
  }, []);

  return {
    isHealthy,
  };
};

export default useHealthCheck;
