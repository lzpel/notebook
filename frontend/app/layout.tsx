import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "notebook",
  description: "手帳を生成する",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="jp">
      <body>
        {children}
      </body>
    </html>
  );
}
