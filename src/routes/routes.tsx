import { Route, Routes } from 'react-router-dom';
import MainContent from '@Components/layout/MainContent';

export interface RouteData {
  component: React.ComponentType;
  path: string;
  icon: React.ComponentType;
  title: string;
  label: string;
}

export const genRoutes = (routesData: RouteData[]) => {
  return (
    <Routes>
      {routesData.map((route) => (
        <Route
          key={route.label}
          path={route.path}
          element={
            <MainContent title={route.title}>
              <route.component />
            </MainContent>
          }
        />
      ))}
      <Route path="*" element={<div>nopage</div>} />
    </Routes>
  );
};
