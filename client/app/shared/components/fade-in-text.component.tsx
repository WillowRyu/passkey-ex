import "../../styles/fade-in-text.css";

interface FadeInTextProps {
  text: string;
}

export const FadeInText = ({ text }: FadeInTextProps) => {
  return (
    <div className="text-base leading-relaxed fade-in-text">
      {text.split("\n").map((line, lineIndex) => (
        <div
          key={lineIndex}
          className="fade-in-line"
          style={{ animationDelay: `${lineIndex * 1}s` }} // 각 줄의 등장 딜레이
        >
          {line.split("").map((char, charIndex) => (
            <span
              key={charIndex}
              style={{
                animationDelay: `${lineIndex * 1 + charIndex * 0.02}s`, // 줄과 글자 딜레이 결합
              }}
              className="fade-in-letter"
            >
              {char}
            </span>
          ))}
        </div>
      ))}
    </div>
  );
};
