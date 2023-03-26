import { Navbar } from '@Components/navbar';
import { genRoutes, RouteData } from '../../routes/routes';

type Props = {
  initialRoutes: RouteData[];
};

const MainLayout: React.FC<Props> = ({ initialRoutes }: Props) => {
  return (
    <div style={{ height: '100vh', display: 'flex' }}>
      <Navbar initialRoutes={initialRoutes} />
      {genRoutes(initialRoutes)}
    </div>
  );
};

export default MainLayout;
