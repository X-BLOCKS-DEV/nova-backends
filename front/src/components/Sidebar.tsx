import { useState } from "react";
import {
  AiOutlineArrowRight,
  AiOutlineArrowLeft,
  AiOutlineFund,
  AiOutlineGold,
  AiOutlineSetting,
} from "react-icons/ai";
import { Link } from "react-router-dom";

function Sidebar() {
  const [open, setOpen] = useState(true);
  const menus = [
    {
      title: "Dashboard",
      path: "/dashboard",
      icon: <AiOutlineFund size={24} />,
      gap: false,
    },
    {
      title: "Reward History",
      path: "/reward-history",
      icon: <AiOutlineGold size={24} />,
      gap: false,
    },
    {
      title: "Settings",
      path: "/settings",
      icon: <AiOutlineSetting size={24} />,
      gap: true,
    },
  ];
  return (
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

      <div className="flex gap-x-2 items-center">
        <img
          src="./src/assets/nova-icon.jpg"
          className="w-16 cursor-pointer rounded-full"
        />
        <h1
          className={`truncate text-white text-md duration-300 origin-left font-medium ${
            !open && "scale-0"
          }`}
        >
          EIZEN 管理画面
        </h1>
      </div>

      <ul className="pt-6 text-white">
        {menus.map((menu, index) => (
          <div className="pl-3 hover:bg-slate-400 rounded">
            <Link to={menu.path}>
              <li
                key={index}
                className={`flex rounded-md p-2 cursor-pointer text-sm items-center gap-x-4 
                ${menu.gap ? "mt-9" : "mt-2"} ${
                  index === 0 && "bg-light-white"
                } `}
              >
                {menu.icon}
                <span
                  className={`${
                    !open && "hidden"
                  } truncate whitespace-nowrap origin-left duration-300`}
                >
                  {menu.title}
                </span>
              </li>
            </Link>
          </div>
        ))}
      </ul>
      <div className="absolute bottom-1 text-slate-300 text-sm duration-300">
        {open ? <p>© 2023 B・C LAB Inc.</p> : ""}
      </div>
    </div>
  );
}

export default Sidebar;
