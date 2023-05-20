import { Outlet } from 'react-router-dom';
import { Navbar } from '@Components/navbar';
import { navlist } from '@Components/navbar/itemList';
import { PropsWithChildren } from 'react';

const MainLayout = ({ children }: PropsWithChildren) => {
  return (
    <div style={{ height: '100vh', display: 'flex' }}>
      <Navbar navList={navlist} />
      {children || <Outlet />}
    </div>
  );
};

export default MainLayout;
