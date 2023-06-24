import { InfoWindowF, MarkerF, MarkerProps } from "@react-google-maps/api";
import { ReactNode, useEffect, useRef, useState } from "react";

type AnimatedMarker = google.maps.Marker & {
  __startPosition_lat: number;
  __startPosition_lng: number;
  __animationHandler: number;
};

function animateMarkerTo(
  marker: AnimatedMarker,
  newPosition: google.maps.LatLng,
  setPosition: (location: google.maps.LatLngLiteral) => void
) {
  const options = {
    duration: 2500,
    easing: function (t: number, b: number, c: number, d: number) {
      // jquery animation: swing (easeOutQuad)
      return -c * (t /= d) * (t - 2) + b;
    },
  };

  // save current position. prefixed to avoid name collisions. separate for lat/lng to avoid calling lat()/lng() in every frame
  marker.__startPosition_lat = marker.getPosition()?.lat() as number;
  marker.__startPosition_lng = marker.getPosition()?.lng() as number;
  let newPosition_lat = newPosition.lat();
  let newPosition_lng = newPosition.lng();

  // crossing the 180° meridian and going the long way around the earth?
  if (Math.abs(newPosition_lng - marker.__startPosition_lng) > 180) {
    if (newPosition_lng > marker.__startPosition_lng) {
      newPosition_lng -= 360;
    } else {
      newPosition_lng += 360;
    }
  }

  const animateStep = function (marker: AnimatedMarker, startTime: number) {
    const ellapsedTime = new Date().getTime() - startTime;
    const durationRatio = ellapsedTime / options.duration; // 0 - 1
    const easingDurationRatio = options.easing(
      ellapsedTime,
      0,
      1,
      options.duration
    );

    if (durationRatio < 1) {
      setPosition({
        lat:
          marker.__startPosition_lat +
          (newPosition_lat - marker.__startPosition_lat) * easingDurationRatio,
        lng:
          marker.__startPosition_lng +
          (newPosition_lng - marker.__startPosition_lng) * easingDurationRatio,
      });

      // use requestAnimationFrame if it exists on this browser. If not, use setTimeout with ~60 fps
      if (window.requestAnimationFrame) {
        marker.__animationHandler = window.requestAnimationFrame(function () {
          animateStep(marker, startTime);
        });
      } else {
        marker.__animationHandler = setTimeout(function () {
          animateStep(marker, startTime);
        }, 17);
      }
    } else {
      setPosition({ lat: newPosition.lat(), lng: newPosition.lng() });
    }
  };

  // stop possibly running animation
  if (window.cancelAnimationFrame) {
    window.cancelAnimationFrame(marker.__animationHandler);
  } else {
    clearTimeout(marker.__animationHandler);
  }

  animateStep(marker, new Date().getTime());
}

type AnimatedMarkerProps = MarkerProps & {
  infoWindowContents: ReactNode;
};

export const AnimatedMarker: React.FC<AnimatedMarkerProps> = ({
  infoWindowContents,
  ...props
}) => {
  const markerRef = useRef<google.maps.Marker>();
  const [position, setPosition] = useState(props.position);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    animateMarkerTo(
      markerRef.current as AnimatedMarker,
      props.position as google.maps.LatLng,
      setPosition
    );
  }, [props.position]);

  return (
    <MarkerF
      {...props}
      position={position}
      onClick={(e) => {
        setOpen(true);
        props.onClick?.(e);
      }}
      onLoad={(marker) => (markerRef.current = marker)}
    >
      {open && (
        <InfoWindowF onCloseClick={() => setOpen(false)} position={position}>
          {infoWindowContents}
        </InfoWindowF>
      )}
    </MarkerF>
  );
};
