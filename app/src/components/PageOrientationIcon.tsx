import { HtmlHTMLAttributes } from "react";

export default function PageOrientationIcon(
  props: HtmlHTMLAttributes<SVGElement>
) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {...props}>
      <path d="M11,3H5A2.002,2.002,0,0,0,3,5V19a2.002,2.002,0,0,0,2,2h6a2.002,2.002,0,0,0,2-2V5A2.002,2.002,0,0,0,11,3ZM5,19V5h6l.00146,14Zm14-8H15a1,1,0,0,0,0,2h4v6H15a1,1,0,0,0,0,2h4a2.002,2.002,0,0,0,2-2V13A2.002,2.002,0,0,0,19,11ZM15,7a2.002,2.002,0,0,1,2,2,1,1,0,0,0,2,0,4.00458,4.00458,0,0,0-4-4,1,1,0,0,0,0,2Z" />
    </svg>
  );
}
