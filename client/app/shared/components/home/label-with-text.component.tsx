interface LabelWithTextProps {
  label: string;
  text: string;
  onClick?: () => void;
}

export function LabelWithText({ label, text, onClick }: LabelWithTextProps) {
  return (
    <div className="flex items-center justify-center bg-transparent w-full">
      <div className="relative w-full min-h-[130px] bg-transparent flex items-end justify-center overflow-hidden">
        <div className="absolute top-[30px] left-[30px] h-[48px] bg-[#1f7cff] text-white font-bold text-2xl py-2 px-6 -rotate-12">
          {label}

          <div className="w-screen absolute left-0 h-0.5 bg-[#1f7cff] translate-y-[12px]" />
        </div>

        <div
          className={`text-center text-white flex overflow-hidden ${
            onClick ? "hover:cursor-pointer" : ""
          }`}
          onClick={onClick}
        >
          <div
            className={`text-2xl font-semibold whitespace-nowrap overflow-hidden text-ellipsis px-2 rounded-md ${
              onClick ? "transition-all duration-200 hover:bg-[#1f7cff]" : ""
            }`}
          >
            {text}
          </div>
        </div>
      </div>
    </div>
  );
}
