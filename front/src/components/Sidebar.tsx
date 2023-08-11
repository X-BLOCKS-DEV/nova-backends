import { useState } from "react";
import { AiOutlineArrowRight, AiOutlineArrowLeft, AiOutlineFund, AiOutlineGold, AiOutlineSetting } from "react-icons/ai";

function Sidebar() {
  const [open, setOpen] = useState(true);
  const menus = [
    {
        title: "Dashboard",
        icon: <AiOutlineFund size={24} />,
        gap: false
    },
    {
        title: "Reward History",
        icon: <AiOutlineGold size={24} />,
        gap: false
    },
    {
        title: "Settings",
        icon: <AiOutlineSetting size={24} />,
        gap: true
    },
  ]
  return (
    <div className="flex">
      <div
        className={`${
          open ? "w-60" : "w-20"
        } h-screen bg-slate-500 relative duration-300 p-2`}
      >
        <div
          className="absolute p-2 bottom-9 -right-6 w-15 h-15 bg-white rounded-full cursor-pointer border-2 border-slate-500"
          onClick={() => setOpen(!open)}
        >
          {open ? (
            <AiOutlineArrowLeft size={30} />
          ) : (
            <AiOutlineArrowRight size={30} />
          )}
        </div>

        <div className="flex gap-x-4 items-center">
          <img
            src="./src/assets/nova-icon.jpg"
            className="w-16 cursor-pointer duration-500 rounded-full"
          />
          <h1 className={`truncate text-white text-lg duration-300 origin-left font-medium ${!open && "scale-0"}`}>
            Nova 管理画面
          </h1>
        </div>

        <ul className="pt-6 text-white">
          {menus.map((menu, index) => (
            <div className="pl-3 hover:bg-slate-400 rounded">
                <li
                key={index}
                className={`flex rounded-md p-2 cursor-pointer text-sm items-center gap-x-4 
                ${menu.gap ? "mt-9" : "mt-2"} ${
                    index === 0 && "bg-light-white"
                } `}
                >
                {menu.icon}
                <span className={`${!open && "hidden"} origin-left duration-300`}>
                    {menu.title}
                </span>
                </li>
            </div>
          ))}
        </ul>

      </div>

      <div className="p-7 text-2xl font-semibold flex-1 h-screen">
        <h1>Home Page</h1>
      </div>
    </div>
  );
}

export default Sidebar;
