// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next';
import path from 'path';
import fs from 'fs';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
	if (req.method === 'GET') {
		res.setHeader('Content-Type', 'text/csv');
		res.setHeader('Content-Disposition', 'attachment; filename=results.csv');
		const fpath = path.resolve(process.cwd(), 'results.csv');
		const fileBuffer = fs.readFileSync(fpath, { encoding: 'utf8' });
		res.send(fileBuffer);
	}
}
