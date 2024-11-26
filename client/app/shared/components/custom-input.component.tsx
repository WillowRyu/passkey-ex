import { InputHTMLAttributes, useRef } from "react";
import "../../styles/custom-input.css";

interface CustomInputProps extends InputHTMLAttributes<HTMLInputElement> {
  title: string;
  subText: string;
}

export function CustomInput({ title, subText, ...props }: CustomInputProps) {
  const labelRef = useRef<HTMLDivElement>(null);
  const onFocus = () => {
    labelRef.current?.classList.add("active");
  };

  const onBlur = () => {
    labelRef.current?.classList.remove("active");
  };

  return (
    <div className="custom_input flex flex-col h-[200px] px-4 rounded-lg">
      <div className="signature-label" ref={labelRef}>
        <span className="line-effect"></span>
        <span className="signature-text">{title}</span>
        <span className="line-effect"></span>
      </div>
      <p className="instruction">{subText}</p>
      <input
        {...props}
        type="text"
        onFocus={onFocus}
        onBlur={onBlur}
        className="signature-input"
        placeholder={props.placeholder ?? ""}
      />
    </div>
  );
}
