import { FadeInText } from "./fade-in-text.component";

interface BottomTextUiProps {
  text: string;
}

export const BottomTextUi = ({ text }: BottomTextUiProps) => {
  return (
    <div className="relative w-[500px] p-6 bg-blue-900/80 text-white rounded-lg shadow-lg text-1xl mb-6">
      <div className="absolute -top-4 left-4 flex items-center bg-blue-800 text-white text-sm font-bold px-3 py-1 rounded-md clip-path-polygon">
        <span className="text-cyan-400">Passkey Admin</span>
        <span className="ml-2 text-white/80 text-xs">TALK</span>
      </div>

      <FadeInText text={text} />
    </div>
  );
};
