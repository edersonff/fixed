import { motion } from "framer-motion";

export function LoadingDots({ label = "Loading" }: { label?: string }) {

  return (

    <span className="loading-status" aria-label={label}>

      <span className="sr-only">{label}</span>

      <span className="loading-dots" aria-hidden="true">

        {[0, 1, 2].map((index) => (

          <motion.span

            key={index}

            animate={{ opacity: [0.25, 1, 0.25], y: [0, -3, 0] }}

            transition={{ duration: 0.4, delay: index * 0.12, repeat: Infinity, ease: "easeInOut" }}

          />

        ))}

      </span>

    </span>

  );

}
