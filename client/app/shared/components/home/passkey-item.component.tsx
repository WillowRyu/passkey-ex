import { CredentialsResponse } from "~/models/credentials_model";

interface PasskeyItemProps {
  credItems: Array<CredentialsResponse>;
  renameItem: (id: string, name: string) => void;
  deleteItem: (id: string) => void;
}

export function PasskeyItemList({
  credItems,
  renameItem,
  deleteItem,
}: PasskeyItemProps) {
  return (
    <div className="bg-transparent flex justify-center max-h-[134px] pl-[15px]">
      <div className="bg-gray-800 text-teal-300 rounded-lg shadow-lg w-full overflow-scroll">
        <ul className="divide-y divide-gray-700">
          {credItems?.map((item) => (
            <li
              key={item.id}
              className="flex justify-between items-center px-4 py-2"
            >
              <span className="bg-gray-700 px-3 py-1 rounded-md text-sm">
                {item.name}
              </span>
              <div className="flex gap-2">
                <div
                  className="text-xl hover:cursor-pointer font-semibold hover:text-teal-200 hover:translate-y-[-2px] transition-all duration-200"
                  onClick={() => renameItem(item.id, item.name)}
                >
                  Rename
                </div>

                <div
                  className="text-xl hover:cursor-pointer font-semibold text-red-500 hover:text-red-400 hover:translate-y-[-2px] transition-all duration-200"
                  onClick={() => deleteItem(item.id)}
                >
                  Delete
                </div>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
