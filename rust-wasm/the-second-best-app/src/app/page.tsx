"use client";

import { useEffect, useState } from "react";
import hello, { draw_fractal, greet } from "../../pkg";

export default function Home() {
  const [message, setMessage] = useState("");
  const [img, setImg] = useState("");

  useEffect(() => {
    hello().then(() => {
      setMessage(greet("Next.js and WebAssembly"));
      let test = draw_fractal();

      // convert this byte array to base64
      let base64 = btoa(
        new Uint8Array(test).reduce(
          (data, byte) => data + String.fromCharCode(byte),
          ""
        )
      );
      setImg(base64);
    });
  }, []);

  return (
    <div>
      <h1>{message}</h1>
      <img src={"data:image/png;base64," + img} />
    </div>
  );
}
