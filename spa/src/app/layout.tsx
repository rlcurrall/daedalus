import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark bg-zinc-800">
      <head>
        <script
          src="https://kit.fontawesome.com/ff80b2ed10.js"
          crossOrigin="anonymous"
          async
        ></script>
      </head>
      <body className={inter.className}>{children}</body>
    </html>
  );
}
